
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::twist::BLS12381TwistCurve;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::unsigned_integer::element::U384;
use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::PointFormat;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::Endianness;

fn main() {
    let gen = BLS12381TwistCurve::generator();

    let secret: U384 = U384::from_hex_unchecked("6C616D6264617370");

    let res = gen.operate_with_self(secret);
    let hex_value = hex::encode(res.serialize(PointFormat::Uncompressed, Endianness::LittleEndian));
    println!("public key in uncompressed short weierstrass projective point little endian is: {:?}" , hex_value);
}
