pub enum SizingMode {
    //Percent, // not implemented yet!
    Dynamic
}

pub enum SegmentType {
    CommandRunner,
    Static
}

pub enum SpacerSize {
    Auto,
    FixedWidth(u64)
}

pub struct DynSpacer {
    sizing_mode: SpacerSize
}

pub struct Segment {
}

pub const SEGMENT_SIZING: SizingMode = SizingMode::Dynamic;

pub const UPDATE_MS: u64 = 1000;
