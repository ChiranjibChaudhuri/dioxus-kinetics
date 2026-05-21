#![forbid(unsafe_code)]

use ui_composition::Composition;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewportProfile {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

impl ViewportProfile {
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            name: name.into(),
            width,
            height,
        }
    }

    pub fn desktop() -> Self {
        Self::new("desktop", 1440, 960)
    }

    pub fn tablet() -> Self {
        Self::new("tablet", 1024, 768)
    }

    pub fn mobile() -> Self {
        Self::new("mobile", 390, 844)
    }

    pub fn validate(&self) -> Result<(), CaptureError> {
        if self.width == 0 || self.height == 0 {
            Err(CaptureError::InvalidViewport)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureMark {
    pub name: String,
    pub frame: u32,
}

impl CaptureMark {
    pub fn new(name: impl Into<String>, frame: u32) -> Self {
        Self {
            name: name.into(),
            frame,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureStageDescriptor {
    pub id: String,
    pub composition_id: String,
}

impl CaptureStageDescriptor {
    pub fn new(id: impl Into<String>, composition_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            composition_id: composition_id.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportManifest {
    pub schema_version: u32,
    pub library_version: String,
    pub compositions: Vec<Composition>,
    pub stages: Vec<CaptureStageDescriptor>,
    pub viewports: Vec<ViewportProfile>,
    pub marks: Vec<CaptureMark>,
}

impl ExportManifest {
    pub fn new(library_version: impl Into<String>) -> Self {
        Self {
            schema_version: 1,
            library_version: library_version.into(),
            compositions: Vec::new(),
            stages: Vec::new(),
            viewports: Vec::new(),
            marks: Vec::new(),
        }
    }

    pub fn with_composition(mut self, composition: Composition) -> Self {
        self.compositions.push(composition);
        self
    }

    pub fn with_stage(mut self, stage: CaptureStageDescriptor) -> Self {
        self.stages.push(stage);
        self
    }

    pub fn with_viewport(mut self, viewport: ViewportProfile) -> Self {
        self.viewports.push(viewport);
        self
    }

    pub fn with_mark(mut self, mark: CaptureMark) -> Self {
        self.marks.push(mark);
        self
    }

    pub fn mark_frame(&self, name: &str) -> Option<u32> {
        self.marks
            .iter()
            .find(|mark| mark.name == name)
            .map(|mark| mark.frame)
    }

    pub fn validate(&self) -> Result<(), CaptureError> {
        for viewport in &self.viewports {
            viewport.validate()?;
        }
        for composition in &self.compositions {
            composition
                .validate()
                .map_err(|_| CaptureError::InvalidComposition)?;
        }
        for stage in &self.stages {
            if !self
                .compositions
                .iter()
                .any(|composition| composition.id == stage.composition_id)
            {
                return Err(CaptureError::MissingComposition);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureError {
    InvalidViewport,
    InvalidComposition,
    MissingComposition,
}
