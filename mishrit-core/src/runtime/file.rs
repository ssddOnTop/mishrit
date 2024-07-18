#[async_trait::async_trait]
pub trait FileIO: Send + Sync {
    async fn write<'a>(&'a self, path: &'a str, content: &'a [u8]) -> anyhow::Result<()>;
    async fn read<'a>(&'a self, path: &'a str) -> anyhow::Result<Vec<u8>>;
}

#[cfg(test)]
pub mod test {
    use super::*;
    use async_trait::async_trait;

    #[derive(Default)]
    pub struct FileIOMock;

    #[async_trait]
    impl FileIO for FileIOMock {
        async fn write<'a>(&'a self, path: &'a str, content: &'a [u8]) -> anyhow::Result<()> {
            std::fs::write(path, content)?;
            Ok(())
        }

        async fn read<'a>(&'a self, path: &'a str) -> anyhow::Result<Vec<u8>> {
            Ok(std::fs::read(path)?)
        }
    }
}
