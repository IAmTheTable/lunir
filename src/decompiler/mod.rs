use crate::il::IlChunk;

pub struct DecompilerSettings {}

#[derive(Default)]
pub struct DecompilerBuilder {
    settings: Option<DecompilerSettings>,
}

impl DecompilerBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_settings(settings: DecompilerSettings) -> Self {
        Self {
            settings: Some(settings),
        }
    }

    fn settings(&mut self, settings: DecompilerSettings) -> &mut Self {
        self.settings = Some(settings);

        self
    }
}

pub struct Decompiler {
    settings: DecompilerSettings,
}

/// The format of bytecode specific to a certain Lua version.
pub trait BytecodeFormat {
    fn serialize(&self, chunk: IlChunk) -> Vec<u8>;
    fn deserialize(&self, bytecode: impl AsRef<[u8]>) -> IlChunk;
}

impl Decompiler {
    /// Begins a decompilation job of bytecode with a specified format.
    pub fn decompile(bytecode: impl AsRef<[u8]>, fmt: impl BytecodeFormat) -> String {
        let _il = fmt.deserialize(bytecode);
        todo!()
    }
}
