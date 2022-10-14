#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Game {
    pub dimensions: Dimensions,
    pub name: String,
    pub is_expansion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Bin {
    pub name: &'static str,
    pub dimensions: Dimensions,
    pub games: Vec<Game>,
}

impl Bin {
    pub fn new(name: &'static str, dimensions: Dimensions) -> Self {
        let games = Vec::new();
        Bin {
            name,
            dimensions,
            games,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub allow_depth_overflow: bool,
    pub vertical: bool,
}

impl Dimensions {
    fn longest_side(self) -> u32 {
        if self.height >= self.width && self.height >= self.depth {
            return self.height;
        }
        if self.width >= self.height && self.width >= self.depth {
            return self.width;
        }
        return self.depth;
    }

    fn get_smallest_sides(self) -> (u32, u32) {
        let longest = self.longest_side();
        let (side_1, side_2): (u32, u32);

        if self.height == longest {
            (side_1, side_2) = (self.width, self.depth);
        } else if self.width == longest {
            (side_1, side_2) = (self.height, self.depth);
        } else {
            (side_1, side_2) = (self.height, self.width);
        }

        if side_1 >= side_2 {
            return (side_1, side_2);
        }

        return (side_2, side_1);
    }
}

impl Game {
    fn dimensions_without_biggest(&self, options: Options) -> Dimensions {
        let (side_1, side_2) = self.dimensions.get_smallest_sides();
        if options.vertical {
            return Dimensions {
                height: side_1,
                width: side_2,
                depth: 1,
            };
        }

        return Dimensions {
            height: side_2,
            width: side_1,
            depth: 1,
        };
    }

    pub fn effective_dimensions_for_bins(&self, bin: &Bin, options: Options) -> Dimensions {
        if !options.allow_depth_overflow {
            // if longest dimension is longer than bin depth, we need to force that dimension to be used
            let longest_side = self.dimensions.longest_side();
            if longest_side > bin.dimensions.depth {
                return Dimensions {
                    height: self.dimensions.get_smallest_sides().1,
                    width: longest_side,
                    depth: 1,
                };
            }
        }

        // Find smallest dimensions and ignore bin depth
        return self.dimensions_without_biggest(options);
    }
}
