
extern crate charcode;

use charcode::traits::CharCoder;
use charcode::reverse::ReverseMessage;
use charcode::substitution::Sub;
use charcode::verify::Verify;

fn main() {
    let orig_msg = r#"
;O dkugrfpfgku fpym fky gxfk fhg feam yg frumyp; U

.jiuraf jggfp; gly gu fpldut ii'lyj dkurrfld m'U yr
,spacgry; skycfr fhg yg paiumur ru fky ruhG
<.> frfhg feam yg gilcuttus wyh wyke g'kys U wyK
"#;
    // First I think we need to revre the contents of the lines.
    let mut rw = ReverseMessage::new();
    println!("{}", rw.decode(orig_msg));

    let key   = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ';'];
    let alpha = vec!['a', 'x', 'c', 'g', 'k', 'e', 't', 'h', 'l', 'y', 'n', 'u', 'm', 'x', ';', 'r', 'x', 's', 'd', 'f', 'i', 'x', 'w', 'x', 'o', 'x', 'p'];

    let mut s = Sub::new(
        &alpha,
        &key
    );

    let step = s.decode(orig_msg);
    let sln = rw.decode(step.as_ref());
    let mut v = Verify::new();
    println!("verified: {}", v.verify(sln.as_ref()));

    println!("{}", sln);

}


