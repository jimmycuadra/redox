extern crate redox;

use redox::describe;

#[test]
fn test() {
    describe("redox", |g| {
        g.it("can pass", || {
            true
        });

        g.it("can fail", || {
            false
        });
    });
}
