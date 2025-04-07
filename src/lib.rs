pub trait Ancestor {
    type A;
}

pub trait Red: Ancestor<A = <Self as Red>::R> {
    type R;
}

pub trait Blue: Ancestor<A = <Self as Blue>::B> {
    type B;
}

pub trait Green: Ancestor<A = <Self as Green>::G> {
    type G;
}

pub trait RedBlue: Red<R = <Self as RedBlue>::RB> + Blue<B = <Self as RedBlue>::RB> {
    type RB;
}

pub trait RedGreen: Red<R = <Self as RedGreen>::RG> + Green<G = <Self as RedGreen>::RG> {
    type RG;
}

pub trait RedBlueGreen:
    RedBlue<RB = <Self as RedBlueGreen>::RBG> + RedGreen<RG = <Self as RedBlueGreen>::RBG>
{
    type RBG;
}
