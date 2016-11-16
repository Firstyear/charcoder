extern crate charcode;

use charcode::traits::CharCoder;
use charcode::substitution::Sub;
use charcode::verify::Verify;

fn main() {
    let orig_msg = r#"
vg vf abg gung rnfl zl sevraq

Xfmdpnf up uif Ljxjdpo Y cbehf dibmmbohf.
Podf bhbjo xf mfbe zpv epxo b qbui pg qvaamft, uifsf jt b qsjaf ps uxp boe uijoht up gjoe. Jg zpv mppl bspvoe boe ibwf b hppe uijol zpv tipvme tubz po uif sjhiu usbdl.

Up sfhjtufs zpv offe up tfoe b uyu up "dpouspm" po Afsp Uxp Tfwfo Pof Afsp Afsp Uxp Gpvs Tjy Gjwf Ojof xjui zpvs qmbzfs dpef. Zpvs qmbzfs dpef jt uif dpef po zpvs cbehf uibu tubsu xjui b Y boe ibt 5 ejhjut bgufs ju. Cf tvsf up ufyu uif gvmm dpef jodmvejoh uif Y.

Tpssz, cvu xf dbo'u tvqqpsu joufsobujpobm qipof ovncfst, J fyqfdu zpv dbo hfu b mpdbm TJN ps qipof gps difbq.

Pwfs uif ofyu gfx ebzt zpv njhiu dpnf bdsptt ibsexbsf xf ibwf cvjmu, qmfbtf epo'u npwf ps upvdi uif ibsexbsf.

Fbdi qvaamf jt tpmwfe xifo zpv ibwf b dpef up uyu up "dpouspm". Bmm dpeft tubsu xjui bo Y, "dpouspm" xjmm pomz sftqpoe jg zpv ibwf b wbmje dpef.

Qvaamf dpeft hfu zpv qpjout, qfstpo xjui uif nptu qpjout xjot.

Ufdi tvqqpsu gps boz jttvft ps gbvmut tipvme cf epof cz fnbjm up ljxjdpocbehf@hnbjm.dpn boe tpnfpof xjmm hfu cbdl up zpv.

Ibwf gvo, cvu lffq jo njoe uif pggjdbm Ljxjdpo dpef pg dpoevdu; UMES epou cf b ejdl.

Evcjpvt Puufs
    "#;

    let key   = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let alpha = vec!['z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y'];

    let mut s = Sub::new(
        &alpha,
        &key
    );

    let sln = s.decode(orig_msg);
    let mut v = Verify::new();
    println!("verified: {}", v.verify(sln.as_ref()));

    println!("{}", sln);
}
