pub mod attribute_info;
pub mod constant_pool;
pub mod field_info;
pub mod method_info;

use crate::class_parser::attribute_info::{parse_attribute_info, AttributeInfo};
use crate::class_parser::constant_pool::{parse_const_pool_info, ConstPool};
use crate::class_parser::field_info::{parse_field_info, FieldInfo};
use crate::class_parser::method_info::{parse_method_info, MethodInfo};
use crate::nom_utils::length_many;
use anyhow::{ensure, Result};
use nom::bytes::complete::tag;
use nom::eof;
use nom::multi::many_m_n;
use nom::number::complete::be_u16;
use nom::IResult;

const MAGIC_NUMBER: u32 = 0xCAFEBABE;

const ACC_PUBLIC: u16 = 0x0001;
const ACC_PRIVATE: u16 = 0x0002;
const ACC_PROTECTED: u16 = 0x0004;
const ACC_STATIC: u16 = 0x0008;
const ACC_FINAL: u16 = 0x0010;
const ACC_SYNCHRONIZED: u16 = 0x0020;
const ACC_SUPER: u16 = 0x0020;
const ACC_BRIDGE: u16 = 0x0040;
const ACC_VOLATILE: u16 = 0x0040;
const ACC_VARARGS: u16 = 0x0080;
const ACC_TRANSIENT: u16 = 0x0080;
const ACC_NATIVE: u16 = 0x0100;
const ACC_INTERFACE: u16 = 0x0200;
const ACC_ABSTRACT: u16 = 0x0400;
const ACC_STRICT: u16 = 0x0800;
const ACC_SYNTHETIC: u16 = 0x1000;
const ACC_ANNOTATION: u16 = 0x2000;
const ACC_ENUM: u16 = 0x4000;

#[derive(Debug)]
struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstPool,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
}

fn is_bit_set(num: u16, flag: u16) -> bool {
    num & flag != 0
}

fn is_bit_clear(num: u16, flag: u16) -> bool {
    num & flag == 0
}

impl ClassFile {
    pub fn new(
        minor_version: u16,
        major_version: u16,
        constant_pool: ConstPool,
        access_flags: u16,
        this_class: u16,
        super_class: u16,
        interfaces: Vec<u16>,
        fields: Vec<FieldInfo>,
        methods: Vec<MethodInfo>,
        attributes: Vec<AttributeInfo>,
    ) -> Result<ClassFile> {
        let class_file = ClassFile {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        };
        class_file.validate_access_flags()?;
        class_file.validate_this_class()?;
        //        class_file.validate_super_class()?;
        class_file.validate_interfaces()?;

        Ok(class_file)
    }

    fn validate_access_flags(&self) -> Result<()> {
        let access_flags = self.access_flags;
        if is_bit_set(access_flags, ACC_INTERFACE) {
            ensure!(
                is_bit_set(access_flags, ACC_ABSTRACT),
                "ACC_ABSTRACT is set"
            );
            ensure!(
                is_bit_clear(access_flags, ACC_FINAL)
                    && is_bit_clear(access_flags, ACC_SUPER)
                    && is_bit_clear(access_flags, ACC_ENUM),
                "access flags"
            );
        } else {
            ensure!(is_bit_clear(access_flags, ACC_ANNOTATION), "access flags");
            ensure!(
                !(is_bit_set(access_flags, ACC_FINAL) && is_bit_set(access_flags, ACC_ABSTRACT)),
                "access flags"
            );
        }
        Ok(())
    }

    fn validate_this_class(&self) -> Result<()> {
        ensure!(
            self.constant_pool.is_class_at(self.this_class),
            "validate this class"
        );
        Ok(())
    }

    fn validate_super_class(&self) -> Result<()> {
        if self.super_class == 0 {
            return Ok(());
        }
        ensure!(
            self.constant_pool.is_class_at(self.this_class),
            "validate super class"
        );
        Ok(())
    }

    fn validate_interfaces(&self) -> Result<()> {
        for interface in &self.interfaces {
            ensure!(
                self.constant_pool.is_class_at(*interface),
                "interface is not class"
            );
        }
        Ok(())
    }
}

fn parse_class_file(buf: &[u8]) -> IResult<&[u8], ClassFile> {
    let (left, _) = tag(&MAGIC_NUMBER.to_be_bytes()[..])(buf)?;
    let (left, minor_version) = be_u16(left)?;
    let (left, major_version) = be_u16(left)?;
    let (left, constant_pool_count) = be_u16(left)?;
    let (left, constant_pool_infos) =
        parse_items(constant_pool_count, parse_const_pool_info, left)?;
    let constant_pool = ConstPool::new(constant_pool_infos);
    let (left, access_flags) = be_u16(left)?;
    let (left, this_class) = be_u16(left)?;
    let (left, super_class) = be_u16(left)?;
    let (left, interfaces) = length_many(be_u16, be_u16)(left)?;
    let (left, fields) = length_many(be_u16, |buf| parse_field_info(&constant_pool, buf))(left)?;
    let (left, methods) = length_many(be_u16, |buf| parse_method_info(&constant_pool, buf))(left)?;
    let (left, attributes) =
        length_many(be_u16, |buf| parse_attribute_info(&constant_pool, buf))(left)?;
    let (left, _) = eof!(left,)?;

    Ok((
        left,
        ClassFile::new(
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        )
        .expect("parse class file"),
    ))
}

fn parse_items<I, O, F>(size: u16, parser: F, buf: I) -> IResult<I, Vec<O>>
where
    I: Clone + PartialEq,
    F: Fn(I) -> IResult<I, O>,
{
    if size > 0 {
        many_m_n(size as usize - 1, size as usize - 1, parser)(buf)
    } else {
        Ok((buf, Vec::new()))
    }
}

#[cfg(test)]
mod tests {
    use crate::class_parser::parse_class_file;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_parse_class_file() -> anyhow::Result<()> {
        let data = [
            202, 254, 186, 190, 0, 0, 0, 52, 0, 29, 10, 0, 6, 0, 15, 9, 0, 16, 0, 17, 8, 0, 18, 10,
            0, 19, 0, 20, 7, 0, 21, 7, 0, 22, 1, 0, 6, 60, 105, 110, 105, 116, 62, 1, 0, 3, 40, 41,
            86, 1, 0, 4, 67, 111, 100, 101, 1, 0, 15, 76, 105, 110, 101, 78, 117, 109, 98, 101,
            114, 84, 97, 98, 108, 101, 1, 0, 4, 109, 97, 105, 110, 1, 0, 22, 40, 91, 76, 106, 97,
            118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 41, 86, 1, 0, 10,
            83, 111, 117, 114, 99, 101, 70, 105, 108, 101, 1, 0, 9, 77, 97, 105, 110, 46, 106, 97,
            118, 97, 12, 0, 7, 0, 8, 7, 0, 23, 12, 0, 24, 0, 25, 1, 0, 5, 72, 101, 108, 108, 111,
            7, 0, 26, 12, 0, 27, 0, 28, 1, 0, 9, 77, 97, 105, 110, 47, 77, 97, 105, 110, 1, 0, 16,
            106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 1, 0, 16, 106,
            97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109, 1, 0, 3, 111, 117,
            116, 1, 0, 21, 76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116,
            114, 101, 97, 109, 59, 1, 0, 19, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110,
            116, 83, 116, 114, 101, 97, 109, 1, 0, 7, 112, 114, 105, 110, 116, 108, 110, 1, 0, 21,
            40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59,
            41, 86, 0, 33, 0, 5, 0, 6, 0, 0, 0, 0, 0, 2, 0, 1, 0, 7, 0, 8, 0, 1, 0, 9, 0, 0, 0, 29,
            0, 1, 0, 1, 0, 0, 0, 5, 42, 183, 0, 1, 177, 0, 0, 0, 1, 0, 10, 0, 0, 0, 6, 0, 1, 0, 0,
            0, 3, 0, 9, 0, 11, 0, 12, 0, 1, 0, 9, 0, 0, 0, 37, 0, 2, 0, 1, 0, 0, 0, 9, 178, 0, 2,
            18, 3, 182, 0, 4, 177, 0, 0, 0, 1, 0, 10, 0, 0, 0, 10, 0, 2, 0, 0, 0, 5, 0, 8, 0, 6, 0,
            1, 0, 13, 0, 0, 0, 2, 0, 14,
        ];

        let (buf, class) = parse_class_file(&data).expect("parse class");
        assert_debug_snapshot!((buf, class));
        Ok(())
    }
}
