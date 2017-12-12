/*
 * |x0..x4| HEADER - always GAME&
 * |x5 | Version?
 * |x6..x54| Name
 * |x55 | Unknown, padding?
 * |x56..xA4 | Author
 * |xa5..xF7 | Unknown. Number 3 stored at F6?
 * |xF8..xF9 | Width
 * |xFA..xFB | Height
 * |x100 | Window flags 1: (Always starts as 60?)
 * 1: Heading when maximized
 * 128: Hide menu on boot
 * |x101 | Window flags 2:
 * 1: Game to include Menu Bar
 * 2: Maximized on boot
 * |xFC-xFE | 24-bit color border
 */
pub struct Gam {
    name: String,
    author: String,
    width: u16,
    height: u16,
}
