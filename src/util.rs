#[macro_export]
macro_rules! define_code {
    {
        $(consts {$($cval:expr),*})?
        $(globals {$($gname:ident),*})?
        $(locals {$($lname:ident),*})?
        {
            $( $inx:ident $($args:expr),* ; )*
        }
    } => {{
        let builder = CodeObjectBuilder::new()
            $(.consts(vec![$( $cval ),*]))?
            $(.locals(vec![$( Variable::from(stringify!($lname)) ),*]))?
            $(.globals(vec![$( Variable::from(stringify!($gname)) ),*]))?
            ;

        let mut c = vec![
            $(
                Instruction::$inx as u8,
                $( $args , )*
            )*
        ];

        builder.code(c).build().unwrap()
    }};
}
