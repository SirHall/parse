#!/usr/bin/env node

function genOrEnum(size) {
    let letters = "abcdefghijklmnopqrstuvwxyz".toUpperCase().split("").filter((n, i) => i < size);
    let types = letters.map(l => `T${l}`);
    let typeList = types.join(", ");


    return `
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Or${size}< ${typeList} >
    {
        ${letters.map(l => `${l}(T${l}),\n\t`).join("")}
    }
    `;
}


function genOrParser(size) {
    let letters = "abcdefghijklmnopqrstuvwxyz".split("").filter((n, i) => i < size);
    let types = letters.map(l => `Dat${l.toUpperCase()}`);
    let typeList = types.join(", ");


    return `
        #[inline]
        pub fn or${size}<'a, ${typeList}>(
        ${letters.map((l, i) => `\t${l}: impl Parser<'a, ${types[i]}>,\n`).join("")}
        ) -> impl Parser<'a, Or${size}< ${typeList} >>
        {
            move |ind : &ParserInput<'a>| -> POut<'a, Or${size}< ${typeList} >> {
                a(ind).map(|ao| ao.with_val(Or${size}::A(ao.val.clone())))
                ${letters.filter((l, i) => i > 0).map((l, i) => `.or_else(|_| ${l}(ind).map(|ao| ao.with_val(Or${size}::${l.toUpperCase()}(ao.val.clone()))))\n\t    `).join("")}
            }
        }

    `;
}

for (i = 2; i <= 12; i++) console.log(genOrParser(i));