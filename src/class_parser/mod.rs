pub mod attribute_info;
pub mod constant_pool;
pub mod descriptor;
pub mod field_info;
pub mod method_info;
mod nom_utils;

use crate::class_parser::attribute_info::{parse_attribute_info, AttributeInfo};
use crate::class_parser::constant_pool::{parse_const_pool_info, ConstPool, ConstPoolInfo};
use crate::class_parser::field_info::{parse_field_info, FieldInfo};
use crate::class_parser::method_info::{parse_method_info, MethodInfo};
use crate::class_parser::nom_utils::length_many;
use color_eyre::Result;
use eyre::ensure;
use nom::bytes::complete::tag;
use nom::eof;
use nom::number::complete::be_u16;
use nom::IResult;

pub const MAGIC_NUMBER: u32 = 0xCAFE_BABE;

pub const JVM_ACC_PUBLIC: u16 = 0x0001;
pub const JVM_ACC_PRIVATE: u16 = 0x0002;
pub const JVM_ACC_PROTECTED: u16 = 0x0004;
pub const JVM_ACC_STATIC: u16 = 0x0008;
pub const JVM_ACC_FINAL: u16 = 0x0010;
pub const JVM_ACC_SYNCHRONIZED: u16 = 0x0020;
pub const JVM_ACC_SUPER: u16 = 0x0020;
pub const JVM_ACC_BRIDGE: u16 = 0x0040;
pub const JVM_ACC_VOLATILE: u16 = 0x0040;
pub const JVM_ACC_VARARGS: u16 = 0x0080;
pub const JVM_ACC_TRANSIENT: u16 = 0x0080;
pub const JVM_ACC_NATIVE: u16 = 0x0100;
pub const JVM_ACC_INTERFACE: u16 = 0x0200;
pub const JVM_ACC_ABSTRACT: u16 = 0x0400;
pub const JVM_ACC_STRICT: u16 = 0x0800;
pub const JVM_ACC_SYNTHETIC: u16 = 0x1000;
pub const JVM_ACC_ANNOTATION: u16 = 0x2000;
pub const JVM_ACC_ENUM: u16 = 0x4000;
pub const JVM_ACC_MODULE: u16 = 0x8000;

pub const JVM_RECOGNIZED_FIELD_MODIFIERS: u16 = JVM_ACC_PUBLIC
    | JVM_ACC_PRIVATE
    | JVM_ACC_PROTECTED
    | JVM_ACC_STATIC
    | JVM_ACC_FINAL
    | JVM_ACC_VOLATILE
    | JVM_ACC_TRANSIENT
    | JVM_ACC_ENUM
    | JVM_ACC_SYNTHETIC;

#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

pub fn is_bit_set(num: u16, flag: u16) -> bool {
    num & flag != 0
}

pub fn is_bit_clear(num: u16, flag: u16) -> bool {
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
        class_file.validate_super_class()?;
        class_file.validate_interfaces()?;

        Ok(class_file)
    }

    fn validate_access_flags(&self) -> Result<()> {
        let access_flags = self.access_flags;
        if is_bit_set(access_flags, JVM_ACC_INTERFACE) {
            ensure!(
                is_bit_set(access_flags, JVM_ACC_ABSTRACT),
                "ACC_ABSTRACT is set"
            );
            ensure!(
                is_bit_clear(access_flags, JVM_ACC_FINAL)
                    && is_bit_clear(access_flags, JVM_ACC_SUPER)
                    && is_bit_clear(access_flags, JVM_ACC_ENUM),
                "access flags"
            );
        } else {
            ensure!(
                is_bit_clear(access_flags, JVM_ACC_ANNOTATION),
                "access flags"
            );
            ensure!(
                !(is_bit_set(access_flags, JVM_ACC_FINAL)
                    && is_bit_set(access_flags, JVM_ACC_ABSTRACT)),
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

pub fn parse_class_file(buf: &[u8]) -> IResult<&[u8], ClassFile> {
    let (left, _) = tag(&MAGIC_NUMBER.to_be_bytes()[..])(buf)?;
    let (left, minor_version) = be_u16(left)?;
    let (left, major_version) = be_u16(left)?;
    let (left, constant_pool_count) = be_u16(left)?;
    let (left, constant_pool_infos) = parse_constant_pool_infos(constant_pool_count - 1, left)?;
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

fn parse_constant_pool_infos(size: u16, mut buf: &[u8]) -> IResult<&[u8], Vec<ConstPoolInfo>> {
    let mut real_pool = Vec::with_capacity(size as usize);
    let mut count = 0;
    loop {
        if count >= size {
            break;
        }
        let ret = parse_const_pool_info(buf)?;
        buf = ret.0;
        let item = ret.1;
        let should_insert_placeholder = matches!(item, ConstPoolInfo::ConstantLongInfo(_) | ConstPoolInfo::ConstantDoubleInfo(_));
        real_pool.push(item);
        count += 1;
        if should_insert_placeholder {
            real_pool.push(ConstPoolInfo::Placeholder);
            count += 1;
        }
    }
    Ok((buf, real_pool))
}
