use sha2::{Digest, Sha256};

use crate::{Error, Result};

/// 生成唯一文件名
pub fn gen_filename(filename: &str, base_name: Option<&str>) -> String {
    let path = std::path::Path::new(filename);
    let ext_name = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let file_basename = if let Some(base_name) = base_name {
        base_name.to_string()
    } else {
        xid::new().to_string()
    };
    format!("{}.{}", file_basename, ext_name)
}

/// 计算数据的哈希值
pub fn data_hash(data: impl AsRef<[u8]>) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let mut buf = [0u8; 64];
    let hash = base16ct::lower::encode_str(hash.as_slice(), &mut buf).map_err(Error::from)?;

    Ok(hash.to_string())
}
