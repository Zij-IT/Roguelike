// todo: Should this file even exist?
//  Colors can be moved to gui, and camera.rs as well
//  Consoles could be moved to main...

pub mod colors {
    pub const BACKGROUND: (u8, u8, u8) = (17, 0, 22);
    pub const FLOOR: (u8, u8, u8) = (26, 26, 26);
    pub const FOREGROUND: (u8, u8, u8) = (243, 251, 241);
    pub const STAIRS: (u8, u8, u8) = (0, 0, 255);
    pub const WALL_REVEALED: (u8, u8, u8) = (77, 77, 77);
    pub const WALL_VISIBLE: (u8, u8, u8) = (0, 179, 0);
    pub const COBBLESTONE: (u8, u8, u8) = (77, 77, 77);
    pub const TOWN_NPC: (u8, u8, u8) = (102, 102, 0);
    pub const WOOD_WALL: (u8, u8, u8) = (77, 61, 38);
}

pub mod consoles {
    pub const HUD_CONSOLE: usize = 2;
    pub const CHAR_CONSOLE: usize = 1;
    pub const MAP_CONSOLE: usize = 0;
    pub const NUM_OF_CONSOLES: usize = 3;
}
