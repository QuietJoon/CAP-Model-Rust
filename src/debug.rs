#[macro_export]
macro_rules! dp {
  ($buf:expr, $format:expr) => {
    if cfg!(imm) {
      writeln!($buf,$format).unwrap();
    } else {
      println!($format);
    }
  };
  ($buf:expr, $format:expr, $($var:expr),*) => {
    if cfg!(imm) {
      writeln!($buf,$format,$($var),*).unwrap();
    } else {
      println!($format,$($var),*);
    }
  }
}

#[macro_export]
macro_rules! fastp {
  ($buf:expr, $format:expr) => {
    if cfg!(debug_assertions) {
      println!($format);
    } else {
      writeln!($buf,$format).unwrap();
    }
  };
  ($buf:expr, $format:expr, $($var:expr),*) => {
    if cfg!(debug_assertions) {
      println!($format,$($var),*);
    } else {
      writeln!($buf,$format,$($var),*).unwrap();
    }
  }
}

#[macro_export]
macro_rules! dop {
  ($buf:expr, $format:expr) => {
    if cfg!(debug_assertions) {
      print!($format);
    }
  };
  ($buf:expr, $format:expr, $($var:expr),*) => {
    if cfg!(debug_assertions) {
      print!($format,$($var),*);
    }
  }
}

#[macro_export]
macro_rules! dopl {
  ($buf:expr, $format:expr) => {
    if cfg!(debug_assertions) {
      println!($format);
    }
  };
  ($buf:expr, $format:expr, $($var:expr),*) => {
    if cfg!(debug_assertions) {
      println!($format,$($var),*);
    }
  }
}
