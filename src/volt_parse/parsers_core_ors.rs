use super::{
    defs::*,
    parser::{POut, Parser, ParserInput},
};
use crate::volt_parse::defs::Or4;

// All the multi-type ors
// Generated using /scripts/or_gen.js, could probably have just used macros

#[inline]
pub fn or2<'a, DatA, DatB>(a : impl Parser<'a, DatA>, b : impl Parser<'a, DatB>) -> impl Parser<'a, Or2<DatA, DatB>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or2<DatA, DatB>> {
        a(ind)
            .map(|ao| ao.with_val(Or2::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or2::B(ao.val.clone()))))
    }
}

#[inline]
pub fn or3<'a, DatA, DatB, DatC>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
) -> impl Parser<'a, Or3<DatA, DatB, DatC>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or3<DatA, DatB, DatC>> {
        a(ind)
            .map(|ao| ao.with_val(Or3::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or3::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or3::C(ao.val.clone()))))
    }
}

#[inline]
pub fn or4<'a, DatA, DatB, DatC, DatD>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
) -> impl Parser<'a, Or4<DatA, DatB, DatC, DatD>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or4<DatA, DatB, DatC, DatD>> {
        a(ind)
            .map(|ao| ao.with_val(Or4::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or4::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or4::C(ao.val.clone()))))
            .or_else(|_| d(ind).map(|ao| ao.with_val(Or4::D(ao.val.clone()))))
    }
}

#[inline]
pub fn or5<'a, DatA, DatB, DatC, DatD, DatE>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
    e : impl Parser<'a, DatE>,
) -> impl Parser<'a, Or5<DatA, DatB, DatC, DatD, DatE>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or5<DatA, DatB, DatC, DatD, DatE>> {
        a(ind)
            .map(|ao| ao.with_val(Or5::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or5::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or5::C(ao.val.clone()))))
            .or_else(|_| d(ind).map(|ao| ao.with_val(Or5::D(ao.val.clone()))))
            .or_else(|_| e(ind).map(|ao| ao.with_val(Or5::E(ao.val.clone()))))
    }
}

#[inline]
pub fn or6<'a, DatA, DatB, DatC, DatD, DatE, DatF>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
    e : impl Parser<'a, DatE>,
    f : impl Parser<'a, DatF>,
) -> impl Parser<'a, Or6<DatA, DatB, DatC, DatD, DatE, DatF>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or6<DatA, DatB, DatC, DatD, DatE, DatF>> {
        a(ind)
            .map(|ao| ao.with_val(Or6::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or6::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or6::C(ao.val.clone()))))
            .or_else(|_| d(ind).map(|ao| ao.with_val(Or6::D(ao.val.clone()))))
            .or_else(|_| e(ind).map(|ao| ao.with_val(Or6::E(ao.val.clone()))))
            .or_else(|_| f(ind).map(|ao| ao.with_val(Or6::F(ao.val.clone()))))
    }
}

#[inline]
pub fn or7<'a, DatA, DatB, DatC, DatD, DatE, DatF, DatG>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
    e : impl Parser<'a, DatE>,
    f : impl Parser<'a, DatF>,
    g : impl Parser<'a, DatG>,
) -> impl Parser<'a, Or7<DatA, DatB, DatC, DatD, DatE, DatF, DatG>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or7<DatA, DatB, DatC, DatD, DatE, DatF, DatG>> {
        a(ind)
            .map(|ao| ao.with_val(Or7::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or7::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or7::C(ao.val.clone()))))
            .or_else(|_| d(ind).map(|ao| ao.with_val(Or7::D(ao.val.clone()))))
            .or_else(|_| e(ind).map(|ao| ao.with_val(Or7::E(ao.val.clone()))))
            .or_else(|_| f(ind).map(|ao| ao.with_val(Or7::F(ao.val.clone()))))
            .or_else(|_| g(ind).map(|ao| ao.with_val(Or7::G(ao.val.clone()))))
    }
}

#[inline]
pub fn or8<'a, DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
    e : impl Parser<'a, DatE>,
    f : impl Parser<'a, DatF>,
    g : impl Parser<'a, DatG>,
    h : impl Parser<'a, DatH>,
) -> impl Parser<'a, Or8<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or8<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH>> {
        a(ind)
            .map(|ao| ao.with_val(Or8::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or8::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or8::C(ao.val.clone()))))
            .or_else(|_| d(ind).map(|ao| ao.with_val(Or8::D(ao.val.clone()))))
            .or_else(|_| e(ind).map(|ao| ao.with_val(Or8::E(ao.val.clone()))))
            .or_else(|_| f(ind).map(|ao| ao.with_val(Or8::F(ao.val.clone()))))
            .or_else(|_| g(ind).map(|ao| ao.with_val(Or8::G(ao.val.clone()))))
            .or_else(|_| h(ind).map(|ao| ao.with_val(Or8::H(ao.val.clone()))))
    }
}

#[inline]
pub fn or9<'a, DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
    e : impl Parser<'a, DatE>,
    f : impl Parser<'a, DatF>,
    g : impl Parser<'a, DatG>,
    h : impl Parser<'a, DatH>,
    i : impl Parser<'a, DatI>,
) -> impl Parser<'a, Or9<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or9<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI>> {
        a(ind)
            .map(|ao| ao.with_val(Or9::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or9::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or9::C(ao.val.clone()))))
            .or_else(|_| d(ind).map(|ao| ao.with_val(Or9::D(ao.val.clone()))))
            .or_else(|_| e(ind).map(|ao| ao.with_val(Or9::E(ao.val.clone()))))
            .or_else(|_| f(ind).map(|ao| ao.with_val(Or9::F(ao.val.clone()))))
            .or_else(|_| g(ind).map(|ao| ao.with_val(Or9::G(ao.val.clone()))))
            .or_else(|_| h(ind).map(|ao| ao.with_val(Or9::H(ao.val.clone()))))
            .or_else(|_| i(ind).map(|ao| ao.with_val(Or9::I(ao.val.clone()))))
    }
}

#[inline]
pub fn or10<'a, DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
    e : impl Parser<'a, DatE>,
    f : impl Parser<'a, DatF>,
    g : impl Parser<'a, DatG>,
    h : impl Parser<'a, DatH>,
    i : impl Parser<'a, DatI>,
    j : impl Parser<'a, DatJ>,
) -> impl Parser<'a, Or10<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or10<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ>> {
        a(ind)
            .map(|ao| ao.with_val(Or10::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or10::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or10::C(ao.val.clone()))))
            .or_else(|_| d(ind).map(|ao| ao.with_val(Or10::D(ao.val.clone()))))
            .or_else(|_| e(ind).map(|ao| ao.with_val(Or10::E(ao.val.clone()))))
            .or_else(|_| f(ind).map(|ao| ao.with_val(Or10::F(ao.val.clone()))))
            .or_else(|_| g(ind).map(|ao| ao.with_val(Or10::G(ao.val.clone()))))
            .or_else(|_| h(ind).map(|ao| ao.with_val(Or10::H(ao.val.clone()))))
            .or_else(|_| i(ind).map(|ao| ao.with_val(Or10::I(ao.val.clone()))))
            .or_else(|_| j(ind).map(|ao| ao.with_val(Or10::J(ao.val.clone()))))
    }
}

#[inline]
pub fn or11<'a, DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ, DatK>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
    e : impl Parser<'a, DatE>,
    f : impl Parser<'a, DatF>,
    g : impl Parser<'a, DatG>,
    h : impl Parser<'a, DatH>,
    i : impl Parser<'a, DatI>,
    j : impl Parser<'a, DatJ>,
    k : impl Parser<'a, DatK>,
) -> impl Parser<'a, Or11<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ, DatK>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or11<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ, DatK>> {
        a(ind)
            .map(|ao| ao.with_val(Or11::A(ao.val.clone())))
            .or_else(|_| b(ind).map(|ao| ao.with_val(Or11::B(ao.val.clone()))))
            .or_else(|_| c(ind).map(|ao| ao.with_val(Or11::C(ao.val.clone()))))
            .or_else(|_| d(ind).map(|ao| ao.with_val(Or11::D(ao.val.clone()))))
            .or_else(|_| e(ind).map(|ao| ao.with_val(Or11::E(ao.val.clone()))))
            .or_else(|_| f(ind).map(|ao| ao.with_val(Or11::F(ao.val.clone()))))
            .or_else(|_| g(ind).map(|ao| ao.with_val(Or11::G(ao.val.clone()))))
            .or_else(|_| h(ind).map(|ao| ao.with_val(Or11::H(ao.val.clone()))))
            .or_else(|_| i(ind).map(|ao| ao.with_val(Or11::I(ao.val.clone()))))
            .or_else(|_| j(ind).map(|ao| ao.with_val(Or11::J(ao.val.clone()))))
            .or_else(|_| k(ind).map(|ao| ao.with_val(Or11::K(ao.val.clone()))))
    }
}

#[inline]
pub fn or12<'a, DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ, DatK, DatL>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    c : impl Parser<'a, DatC>,
    d : impl Parser<'a, DatD>,
    e : impl Parser<'a, DatE>,
    f : impl Parser<'a, DatF>,
    g : impl Parser<'a, DatG>,
    h : impl Parser<'a, DatH>,
    i : impl Parser<'a, DatI>,
    j : impl Parser<'a, DatJ>,
    k : impl Parser<'a, DatK>,
    l : impl Parser<'a, DatL>,
) -> impl Parser<'a, Or12<DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ, DatK, DatL>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Or12< DatA, DatB, DatC, DatD, DatE, DatF, DatG, DatH, DatI, DatJ, DatK, DatL >> {
        a(ind).map(|ao| ao.with_val(Or12::A(ao.val.clone())))
        .or_else(|_| b(ind).map(|ao| ao.with_val(Or12::B(ao.val.clone()))))
.or_else(|_| c(ind).map(|ao| ao.with_val(Or12::C(ao.val.clone()))))
.or_else(|_| d(ind).map(|ao| ao.with_val(Or12::D(ao.val.clone()))))
.or_else(|_| e(ind).map(|ao| ao.with_val(Or12::E(ao.val.clone()))))
.or_else(|_| f(ind).map(|ao| ao.with_val(Or12::F(ao.val.clone()))))
.or_else(|_| g(ind).map(|ao| ao.with_val(Or12::G(ao.val.clone()))))
.or_else(|_| h(ind).map(|ao| ao.with_val(Or12::H(ao.val.clone()))))
.or_else(|_| i(ind).map(|ao| ao.with_val(Or12::I(ao.val.clone()))))
.or_else(|_| j(ind).map(|ao| ao.with_val(Or12::J(ao.val.clone()))))
.or_else(|_| k(ind).map(|ao| ao.with_val(Or12::K(ao.val.clone()))))
.or_else(|_| l(ind).map(|ao| ao.with_val(Or12::L(ao.val.clone()))))

    }
}
