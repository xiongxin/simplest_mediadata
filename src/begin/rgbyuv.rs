use std::fs;
use std::io::Result;
use std::path::Path;

fn simple_yuv444_split<P: AsRef<Path>>(
    path: P,
    width: usize,
    height: usize,
    number: usize,
) -> Result<()> {
    let res = fs::read(path)?;

    for _ in 0..number {
        fs::write("output/output_444_y.y", &res[0..width * height])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() -> Result<()> {
        simple_yuv444_split(
            "/home/xiongxin/data/Code/simplest_mediadata/asserts/lena_256x256_yuv420p.yuv",
            256,
            256,
            1,
        )?;

        Ok(())
    }
}
