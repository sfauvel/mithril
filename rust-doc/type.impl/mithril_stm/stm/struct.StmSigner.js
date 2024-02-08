(function() {var type_impls = {
"mithril_common":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StmSigner%3CD%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#427\">source</a><a href=\"#impl-StmSigner%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"struct\" href=\"mithril_stm/stm/struct.StmSigner.html\" title=\"struct mithril_stm::stm::StmSigner\">StmSigner</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + Digest + FixedOutput,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.sign\" class=\"method\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#434\">source</a><h4 class=\"code-header\">pub fn <a href=\"mithril_stm/stm/struct.StmSigner.html#tymethod.sign\" class=\"fn\">sign</a>(&amp;self, msg: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"mithril_stm/stm/struct.StmSig.html\" title=\"struct mithril_stm::stm::StmSig\">StmSig</a>&gt;</h4></section></summary><div class=\"docblock\"><p>This function produces a signature following the description of Section 2.4.\nOnce the signature is produced, this function checks whether any index in <code>[0,..,self.params.m]</code>\nwins the lottery by evaluating the dense mapping.\nIt records all the winning indexes in <code>Self.indexes</code>.\nIf it wins at least one lottery, it stores the signer’s merkle tree index. The proof of membership\nwill be handled by the aggregator.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.verification_key\" class=\"method\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#450\">source</a><h4 class=\"code-header\">pub fn <a href=\"mithril_stm/stm/struct.StmSigner.html#tymethod.verification_key\" class=\"fn\">verification_key</a>(&amp;self) -&gt; <a class=\"struct\" href=\"mithril_stm/multi_sig/struct.VerificationKey.html\" title=\"struct mithril_stm::multi_sig::VerificationKey\">VerificationKey</a></h4></section></summary><div class=\"docblock\"><p>Extract the verification key.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_stake\" class=\"method\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#455\">source</a><h4 class=\"code-header\">pub fn <a href=\"mithril_stm/stm/struct.StmSigner.html#tymethod.get_stake\" class=\"fn\">get_stake</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class=\"docblock\"><p>Extract stake from the signer.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.core_sign\" class=\"method\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#464\">source</a><h4 class=\"code-header\">pub fn <a href=\"mithril_stm/stm/struct.StmSigner.html#tymethod.core_sign\" class=\"fn\">core_sign</a>(&amp;self, msg: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u8.html\">u8</a>], total_stake: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u64.html\">u64</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"mithril_stm/stm/struct.StmSig.html\" title=\"struct mithril_stm::stm::StmSig\">StmSig</a>&gt;</h4></section></summary><div class=\"docblock\"><p>A core signature generated without closed registration.\nThe core signature can be verified by core verifier.\nOnce the signature is produced, this function checks whether any index in <code>[0,..,self.params.m]</code>\nwins the lottery by evaluating the dense mapping.\nIt records all the winning indexes in <code>Self.indexes</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.check_lottery\" class=\"method\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#480\">source</a><h4 class=\"code-header\">pub fn <a href=\"mithril_stm/stm/struct.StmSigner.html#tymethod.check_lottery\" class=\"fn\">check_lottery</a>(\n    &amp;self,\n    msg: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u8.html\">u8</a>],\n    sigma: &amp;<a class=\"struct\" href=\"mithril_stm/multi_sig/struct.Signature.html\" title=\"struct mithril_stm::multi_sig::Signature\">Signature</a>,\n    total_stake: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u64.html\">u64</a>\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.u64.html\">u64</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Collects and returns the winning indices.</p>\n</div></details></div></details>",0,"mithril_common::crypto_helper::types::alias::ProtocolSigner"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-StmSigner%3CD%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#178\">source</a><a href=\"#impl-Clone-for-StmSigner%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"mithril_stm/stm/struct.StmSigner.html\" title=\"struct mithril_stm::stm::StmSigner\">StmSigner</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + Digest,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#178\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"mithril_stm/stm/struct.StmSigner.html\" title=\"struct mithril_stm::stm::StmSigner\">StmSigner</a>&lt;D&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.76.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.76.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","mithril_common::crypto_helper::types::alias::ProtocolSigner"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-StmSigner%3CD%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#178\">source</a><a href=\"#impl-Debug-for-StmSigner%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"mithril_stm/stm/struct.StmSigner.html\" title=\"struct mithril_stm::stm::StmSigner\">StmSigner</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + Digest,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/mithril_stm/stm.rs.html#178\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","mithril_common::crypto_helper::types::alias::ProtocolSigner"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()