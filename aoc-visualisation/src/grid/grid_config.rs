use bitflags::bitflags;

bitflags! {
    pub(crate) struct GridCellEdge: u8 {
        const TOP = 0b0001;
        const RIGHT = 0b0010;
        const BOTTOM = 0b0100;
        const LEFT = 0b1000;
        const ALL = 0b1111;
    }
}