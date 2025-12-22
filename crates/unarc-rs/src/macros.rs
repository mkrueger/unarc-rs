macro_rules! convert_u32 {
    ( $t:ident, $x:expr ) => {
        let $t = $x[0] as u32 | ($x[1] as u32) << 8 | ($x[2] as u32) << 16 | ($x[3] as u32) << 24;

        #[allow(unused_assignments)]
        {
            $x = &$x[4..];
        }
    };
}

macro_rules! convert_u16 {
    ( $t:ident, $x:expr ) => {
        let $t = $x[0] as u16 | ($x[1] as u16) << 8;

        #[allow(unused_assignments)]
        {
            $x = &$x[2..];
        }
    };
}
macro_rules! get_i16 {
    ( $x:expr ) => {{
        let res = $x[0] as i16 | ($x[1] as i16) << 8;

        #[allow(unused_assignments)]
        {
            $x = &$x[2..];
        }
        res
    }};
}

macro_rules! convert_u8 {
    ( $t:ident, $x:expr ) => {
        let $t = $x[0];

        #[allow(unused_assignments)]
        {
            $x = &$x[1..];
        }
    };
}

macro_rules! convert_string {
    ( $t:ident, $x:expr ) => {
        let mut $t = String::new();
        while $x[0] != 0 {
            $t.push($x[0] as char);
            $x = &$x[1..];
        }
        #[allow(unused_assignments)]
        {
            $x = &$x[1..];
        }
    };
}

macro_rules! skip {
    ( $x:expr, $t:expr  ) => {
        #[allow(unused_assignments)]
        {
            $x = &$x[$t..];
        }
    };
}
