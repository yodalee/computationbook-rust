use universality_is_everywhere::ski_calculus::ski::{SKI};
use universality_is_everywhere::ski_calculus::skicombinator::{SKICombinator};
use super::iota::{iota};

pub fn to_iota(node: &Box<SKI>) -> Box<SKI> {
    match **node {
        SKI::SKISymbol(_) => node.clone(),
        SKI::SKICall(ref l, ref r) => SKI::skicall(to_iota(l), to_iota(r)),
        SKI::SKICombinator(ref c) => comb_to_iota(c),
    }
}

fn comb_to_iota(node: &Box<SKICombinator>) -> Box<SKI> {
    match node.name() {
        'S' => { SKI::skicall(iota(),
                 SKI::skicall(iota(),
                 SKI::skicall(iota(),
                 SKI::skicall(iota(), iota())))) },
        'K' => { SKI::skicall(iota(),
                 SKI::skicall(iota(),
                 SKI::skicall(iota(), iota()))) },
        'I' => { SKI::skicall(iota(), iota()) },
        'ɩ' => { Box::new(SKI::SKICombinator(node.clone())) },
        _ => unreachable!(),
    }
}
