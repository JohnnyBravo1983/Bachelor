# 🎓 Bachelor – Maplib Datalog Evaluator (Demo)

Dette repoet er en **illustrativ demo** basert på bachelorprosjektet mitt (ferdig juni 2024), der jeg jobbet med en **Rust-basert Datalog-evaluator** med:
- parsing til AST (forenklet i demo)
- omskriving/oversettelse til spørringsplan (SPARQL-inspirert)
- **to cache-prinsipper** (statisk og delta) som skiller mellom ting som kan gjenbrukes over runder

rewriting.rs, cache/ og sparql/ er illustrative snutter og bygges ikke i library’et. Testene kjører kun på semi_naive-delen.

I originalprosjektet oppnådde vi **5–9× raskere spørringer** enn baseline. Hele originalrepoet er privat av hensyn til samarbeidet med **Data Treehouse**, men denne demoen viser **nøkkelelementer jeg skrev** – på generiske, syntetiske data. Domenespesifikke detaljer er fjernet.

> Demoen er selvstendig og kjørbar lokalt (`cargo test`). Koden er liten og pedagogisk, ikke en full motor.

---

## 🧱 Hva finner du her
- `src/semi_naive.rs` – liten funksjon som viser **semi-naiv/iterativ** idé (transitiv lukning for `connected`)
- `src/rewriting.rs` – **statisk/dynamisk omskriving** på et minimalt AST + enkel spørringsplan
- `src/cache/` – et lite **Cache-trait** og en enkel **StaticCache**-implementasjon
- `src/sparql/traits.rs` – skisse for et query-engine-interface (generisk)
- `examples/` – små, syntetiske filer: `rules.dl` og `dataset.ttl`
- `tests/integration.rs` – enkle tester som dokumenterer forventet adferd

---

## ⚙️ Kjøring (Rust)
```bash
# kjør tester
cargo test
