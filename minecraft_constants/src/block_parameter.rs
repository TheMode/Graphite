// DO NOT MANUALLY EDIT THIS FILE
// This file has been autogenerated by minecraft_constants/build.rs
// Data is provided courtesy of `https://github.com/PrismarineJS/minecraft-data`

#[repr(u8)]
#[derive(Debug)]
pub enum Orientation {
	DownEast,
	DownNorth,
	DownSouth,
	DownWest,
	UpEast,
	UpNorth,
	UpSouth,
	UpWest,
	WestUp,
	EastUp,
	NorthUp,
	SouthUp,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Thickness {
	TipMerge,
	Tip,
	Frustum,
	Middle,
	Base,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Part {
	Head,
	Foot,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Facing {
	North,
	East,
	South,
	West,
	Up,
	Down,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Tilt {
	None,
	Unstable,
	Partial,
	Full,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Direction {
	North,
	South,
	West,
	East,
}

#[repr(u8)]
#[derive(Debug)]
pub enum UpperOrLower {
	Upper,
	Lower,
}

#[repr(u8)]
#[derive(Debug)]
pub enum ChestType {
	Single,
	Left,
	Right,
}

#[repr(u8)]
#[derive(Debug)]
pub enum StairShape {
	Straight,
	InnerLeft,
	InnerRight,
	OuterLeft,
	OuterRight,
}

#[repr(u8)]
#[derive(Debug)]
pub enum SlabType {
	Top,
	Bottom,
	Double,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Hinge {
	Left,
	Right,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Face {
	Floor,
	Wall,
	Ceiling,
}

#[repr(u8)]
#[derive(Debug)]
pub enum VerticalDirection {
	Up,
	Down,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Leaves {
	None,
	Small,
	Large,
}

#[repr(u8)]
#[derive(Debug)]
pub enum StraightRailShape {
	NorthSouth,
	EastWest,
	AscendingEast,
	AscendingWest,
	AscendingNorth,
	AscendingSouth,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Axis3D {
	X,
	Y,
	Z,
}

#[repr(u8)]
#[derive(Debug)]
pub enum ComparatorMode {
	Compare,
	Subtract,
}

#[repr(u8)]
#[derive(Debug)]
pub enum WallConnection {
	None,
	Low,
	Tall,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Axis2D {
	X,
	Z,
}

#[repr(u8)]
#[derive(Debug)]
pub enum SculkSensorPhase {
	Inactive,
	Active,
	Cooldown,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Instrument {
	Harp,
	Basedrum,
	Snare,
	Hat,
	Bass,
	Flute,
	Bell,
	Guitar,
	Chime,
	Xylophone,
	IronXylophone,
	CowBell,
	Didgeridoo,
	Bit,
	Banjo,
	Pling,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Half {
	Top,
	Bottom,
}

#[repr(u8)]
#[derive(Debug)]
pub enum PistonType {
	Normal,
	Sticky,
}

#[repr(u8)]
#[derive(Debug)]
pub enum WireConnection {
	Up,
	Side,
	None,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Attachment {
	Floor,
	Ceiling,
	SingleWall,
	DoubleWall,
}

#[repr(u8)]
#[derive(Debug)]
pub enum StructureBlockMode {
	Save,
	Load,
	Corner,
	Data,
}

#[repr(u8)]
#[derive(Debug)]
pub enum DirectionOrDown {
	Down,
	North,
	South,
	West,
	East,
}

#[repr(u8)]
#[derive(Debug)]
pub enum RailShape {
	NorthSouth,
	EastWest,
	AscendingEast,
	AscendingWest,
	AscendingNorth,
	AscendingSouth,
	SouthEast,
	SouthWest,
	NorthWest,
	NorthEast,
}

