//! `RBX::Http` client state (IDA 0xa51fe4): three inline strings plus the
//! header map. `Drop` releases them; the destructor decomp is a clear.

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Http {
 /// Request line head at +0 (IDA 0xa52068).
 pub head: String,
 /// Request tail at +8 (IDA 0xa5205a).
 pub tail: String,
 /// Header table at +12 (IDA 0xa52052 `_M_erase`).
 pub headers: HashMap<String, String>,
 /// Body at +36 (IDA 0xa52012).
 pub body: String,
}

impl Http {
 /// `RBX::Http::~Http` (IDA 0xa51fe4): erases the header map and
 /// releases the three strings.
 pub fn destroy(mut self) {
 self.headers.clear();
 self.head.clear();
 self.tail.clear();
 self.body.clear();
 }
}
