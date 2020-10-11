pub(crate) use self::d3d_compile::{D3DCompile, D3DCompileBuilder};

pub fn d3d_compile<'a>() -> D3DCompileBuilder<'a, ((), (), (), (), (), (), (), ())> {
    D3DCompile::builder()
}

mod d3d_compile;
