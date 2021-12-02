use super::parse_matrix;
use crate::{
	poseidon::{sbox::PoseidonSbox, PoseidonParameters},
	utils::parse_vec,
};
use ark_ff::PrimeField;
use ark_ff::vec::Vec;
pub const FULL_ROUNDS: u8 = 8;
pub const PARTIAL_ROUNDS: u8 = 56;
pub const WIDTH: u8 = 2;
pub const SBOX: PoseidonSbox = PoseidonSbox(5);

pub const ROUND_CONSTS: [&str; 128] = [
	"0x09c46e9ec68e9bd4fe1faaba294cba38a71aa177534cdd1b6c7dc0dbd0abd7a7",
	"0x0c0356530896eec42a97ed937f3135cfc5142b3ae405b8343c1d83ffa604cb81",
	"0x1e28a1d935698ad1142e51182bb54cf4a00ea5aabd6268bd317ea977cc154a30",
	"0x27af2d831a9d2748080965db30e298e40e5757c3e008db964cf9e2b12b91251f",
	"0x1e6f11ce60fc8f513a6a3cfe16ae175a41291462f214cd0879aaf43545b74e03",
	"0x2a67384d3bbd5e438541819cb681f0be04462ed14c3613d8f719206268d142d3",
	"0x0b66fdf356093a611609f8e12fbfecf0b985e381f025188936408f5d5c9f45d0",
	"0x012ee3ec1e78d470830c61093c2ade370b26c83cc5cebeeddaa6852dbdb09e21",
	"0x0252ba5f6760bfbdfd88f67f8175e3fd6cd1c431b099b6bb2d108e7b445bb1b9",
	"0x179474cceca5ff676c6bec3cef54296354391a8935ff71d6ef5aeaad7ca932f1",
	"0x2c24261379a51bfa9228ff4a503fd4ed9c1f974a264969b37e1a2589bbed2b91",
	"0x1cc1d7b62692e63eac2f288bd0695b43c2f63f5001fc0fc553e66c0551801b05",
	"0x255059301aada98bb2ed55f852979e9600784dbf17fbacd05d9eff5fd9c91b56",
	"0x28437be3ac1cb2e479e1f5c0eccd32b3aea24234970a8193b11c29ce7e59efd9",
	"0x28216a442f2e1f711ca4fa6b53766eb118548da8fb4f78d4338762c37f5f2043",
	"0x2c1f47cd17fa5adf1f39f4e7056dd03feee1efce03094581131f2377323482c9",
	"0x07abad02b7a5ebc48632bcc9356ceb7dd9dafca276638a63646b8566a621afc9",
	"0x0230264601ffdf29275b33ffaab51dfe9429f90880a69cd137da0c4d15f96c3c",
	"0x1bc973054e51d905a0f168656497ca40a864414557ee289e717e5d66899aa0a9",
	"0x2e1c22f964435008206c3157e86341edd249aff5c2d8421f2a6b22288f0a67fc",
	"0x1224f38df67c5378121c1d5f461bbc509e8ea1598e46c9f7a70452bc2bba86b8",
	"0x02e4e69d8ba59e519280b4bd9ed0068fd7bfe8cd9dfeda1969d2989186cde20e",
	"0x1f1eccc34aaba0137f5df81fc04ff3ee4f19ee364e653f076d47e9735d98018e",
	"0x1672ad3d709a353974266c3039a9a7311424448032cd1819eacb8a4d4284f582",
	"0x283e3fdc2c6e420c56f44af5192b4ae9cda6961f284d24991d2ed602df8c8fc7",
	"0x1c2a3d120c550ecfd0db0957170fa013683751f8fdff59d6614fbd69ff394bcc",
	"0x216f84877aac6172f7897a7323456efe143a9a43773ea6f296cb6b8177653fbd",
	"0x2c0d272becf2a75764ba7e8e3e28d12bceaa47ea61ca59a411a1f51552f94788",
	"0x16e34299865c0e28484ee7a74c454e9f170a5480abe0508fcb4a6c3d89546f43",
	"0x175ceba599e96f5b375a232a6fb9cc71772047765802290f48cd939755488fc5",
	"0x0c7594440dc48c16fead9e1758b028066aa410bfbc354f54d8c5ffbb44a1ee32",
	"0x1a3c29bc39f21bb5c466db7d7eb6fd8f760e20013ccf912c92479882d919fd8d",
	"0x0ccfdd906f3426e5c0986ea049b253400855d349074f5a6695c8eeabcd22e68f",
	"0x14f6bc81d9f186f62bdb475ce6c9411866a7a8a3fd065b3ce0e699b67dd9e796",
	"0x0962b82789fb3d129702ca70b2f6c5aacc099810c9c495c888edeb7386b97052",
	"0x1a880af7074d18b3bf20c79de25127bc13284ab01ef02575afef0c8f6a31a86d",
	"0x10cba18419a6a332cd5e77f0211c154b20af2924fc20ff3f4c3012bb7ae9311b",
	"0x057e62a9a8f89b3ebdc76ba63a9eaca8fa27b7319cae3406756a2849f302f10d",
	"0x287c971de91dc0abd44adf5384b4988cb961303bbf65cff5afa0413b44280cee",
	"0x21df3388af1687bbb3bca9da0cca908f1e562bc46d4aba4e6f7f7960e306891d",
	"0x1be5c887d25bce703e25cc974d0934cd789df8f70b498fd83eff8b560e1682b3",
	"0x268da36f76e568fb68117175cea2cd0dd2cb5d42fda5acea48d59c2706a0d5c1",
	"0x0e17ab091f6eae50c609beaf5510ececc5d8bb74135ebd05bd06460cc26a5ed6",
	"0x04d727e728ffa0a67aee535ab074a43091ef62d8cf83d270040f5caa1f62af40",
	"0x0ddbd7bf9c29341581b549762bc022ed33702ac10f1bfd862b15417d7e39ca6e",
	"0x2790eb3351621752768162e82989c6c234f5b0d1d3af9b588a29c49c8789654b",
	"0x1e457c601a63b73e4471950193d8a570395f3d9ab8b2fd0984b764206142f9e9",
	"0x21ae64301dca9625638d6ab2bbe7135ffa90ecd0c43ff91fc4c686fc46e091b0",
	"0x0379f63c8ce3468d4da293166f494928854be9e3432e09555858534eed8d350b",
	"0x002d56420359d0266a744a080809e054ca0e4921a46686ac8c9f58a324c35049",
	"0x123158e5965b5d9b1d68b3cd32e10bbeda8d62459e21f4090fc2c5af963515a6",
	"0x0be29fc40847a941661d14bbf6cbe0420fbb2b6f52836d4e60c80eb49cad9ec1",
	"0x1ac96991dec2bb0557716142015a453c36db9d859cad5f9a233802f24fdf4c1a",
	"0x1596443f763dbcc25f4964fc61d23b3e5e12c9fa97f18a9251ca3355bcb0627e",
	"0x12e0bcd3654bdfa76b2861d4ec3aeae0f1857d9f17e715aed6d049eae3ba3212",
	"0x0fc92b4f1bbea82b9ea73d4af9af2a50ceabac7f37154b1904e6c76c7cf964ba",
	"0x1f9c0b1610446442d6f2e592a8013f40b14f7c7722236f4f9c7e965233872762",
	"0x0ebd74244ae72675f8cde06157a782f4050d914da38b4c058d159f643dbbf4d3",
	"0x2cb7f0ed39e16e9f69a9fafd4ab951c03b0671e97346ee397a839839dccfc6d1",
	"0x1a9d6e2ecff022cc5605443ee41bab20ce761d0514ce526690c72bca7352d9bf",
	"0x2a115439607f335a5ea83c3bc44a9331d0c13326a9a7ba3087da182d648ec72f",
	"0x23f9b6529b5d040d15b8fa7aee3e3410e738b56305cd44f29535c115c5a4c060",
	"0x05872c16db0f72a2249ac6ba484bb9c3a3ce97c16d58b68b260eb939f0e6e8a7",
	"0x1300bdee08bb7824ca20fb80118075f40219b6151d55b5c52b624a7cdeddf6a7",
	"0x19b9b63d2f108e17e63817863a8f6c288d7ad29916d98cb1072e4e7b7d52b376",
	"0x015bee1357e3c015b5bda237668522f613d1c88726b5ec4224a20128481b4f7f",
	"0x2953736e94bb6b9f1b9707a4f1615e4efe1e1ce4bab218cbea92c785b128ffd1",
	"0x0b069353ba091618862f806180c0385f851b98d372b45f544ce7266ed6608dfc",
	"0x304f74d461ccc13115e4e0bcfb93817e55aeb7eb9306b64e4f588ac97d81f429",
	"0x15bbf146ce9bca09e8a33f5e77dfe4f5aad2a164a4617a4cb8ee5415cde913fc",
	"0x0ab4dfe0c2742cde44901031487964ed9b8f4b850405c10ca9ff23859572c8c6",
	"0x0e32db320a044e3197f45f7649a19675ef5eedfea546dea9251de39f9639779a",
	"0x0a1756aa1f378ca4b27635a78b6888e66797733a82774896a3078efa516da016",
	"0x044c4a33b10f693447fd17177f952ef895e61d328f85efa94254d6a2a25d93ef",
	"0x2ed3611b725b8a70be655b537f66f700fe0879d79a496891d37b07b5466c4b8b",
	"0x1f9ba4e8bab7ce42c8ecc3d722aa2e0eadfdeb9cfdd347b5d8339ea7120858aa",
	"0x1b233043052e8c288f7ee907a84e518aa38e82ac4502066db74056f865c5d3da",
	"0x2431e1cc164bb8d074031ab72bd55b4c902053bfc0f14db0ca2f97b020875954",
	"0x082f934c91f5aac330cd6953a0a7db45a13e322097583319a791f273965801fd",
	"0x2b9a0a223e7538b0a34be074315542a3c77245e2ae7cbe999ad6bb930c48997c",
	"0x0e1cd91edd2cfa2cceb85483b887a9be8164163e75a8a00eb0b589cc70214e7d",
	"0x2e1eac0f2bfdfd63c951f61477e3698999774f19854d00f588d324601cebe2f9",
	"0x0cbfa95f37fb74060c76158e769d6d157345784d8efdb33c23d748115b500b83",
	"0x08f05b3be923ed44d65ad49d8a61e9a676d991e3a77513d9980c232dfa4a4f84",
	"0x22719e2a070bcd0852bf8e21984d0443e7284925dc0758a325a2dd510c047ef6",
	"0x041f596a9ee1cb2bc060f7fcc3a1ab4c7bdbf036119982c0f41f62b2f26830c0",
	"0x233fd35de1be520a87628eb06f6b1d4c021be1c2d0dc464a19fcdd0986b10f89",
	"0x0524b46d1aa87a5e4325e0a423ebc810d31e078aa1b4707eefcb453c61c9c267",
	"0x2c34f424c81e5716ce47fcac894b85824227bb954b0f3199cc4486237c515211",
	"0x0b5f2a4b63387819207effc2b5541fb72dd2025b5457cc97f33010327de4915e",
	"0x22207856082ccc54c5b72fe439d2cfd6c17435d2f57af6ceaefac41fe05c659f",
	"0x24d57a8bf5da63fe4e24159b7f8950b5cdfb210194caf79f27854048ce2c8171",
	"0x0afab181fdd5e0583b371d75bd693f98374ad7097bb01a8573919bb23b79396e",
	"0x2dba9b108f208772998a52efac7cbd5676c0057194c16c0bf16290d62b1128ee",
	"0x26349b66edb8b16f56f881c788f53f83cbb83de0bd592b255aff13e6bce420b3",
	"0x25af7ce0e5e10357685e95f92339753ad81a56d28ecc193b235288a3e6f137db",
	"0x25b4ce7bd2294390c094d6a55edd68b970eed7aae88b2bff1f7c0187fe35011f",
	"0x22c543f10f6c89ec387e53f1908a88e5de9cef28ebdf30b18cb9d54c1e02b631",
	"0x0236f93e7789c4724fc7908a9f191e1e425e906a919d7a34df668e74882f87a9",
	"0x29350b401166ca010e7d27e37d05da99652bdae114eb01659cb497af980c4b52",
	"0x0eed787d65820d3f6bd31bbab547f75a65edb75d844ebb89ee1260916652363f",
	"0x07cc1170f13b46f2036a753f520b3291fdcd0e99bd94297d1906f656f4de6fad",
	"0x22b939233b1d7205f49bcf613a3d30b1908786d7f9f5d10c2059435689e8acea",
	"0x01451762a0aab81c8aad1dc8bc33e870740f083a5aa85438add650ace60ae5a6",
	"0x23506bb5d8727d4461fabf1025d46d1fe32eaa61dec7da57e704fec0892fce89",
	"0x2e484c44e838aea0bac06ae3f71bdd092a3709531e1efea97f8bd68907355522",
	"0x0f4bc7d07ebafd64379e78c50bd2e42baf4a594545cedc2545418da26835b54c",
	"0x1f4d3c8f6583e9e5fa76637862faaee851582388725df460e620996d50d8e74e",
	"0x093514e0c70711f82660d07be0e4a988fae02abc7b681d9153eb9bcb48fe7389",
	"0x1adab0c8e2b3bad346699a2b5f3bc03643ee83ece47228f24a58e0a347e153d8",
	"0x1672b1726057d99dd14709ebb474641a378c1b94b8072bac1a22dbef9e80dad2",
	"0x1dfd53d4576af2e38f44f53fdcab468cc5d8e2fae0acc4ee30d47b239b479c14",
	"0x0c6888a10b75b0f3a70a36263a37e17fe6d77d640f6fc3debc7f207753205c60",
	"0x1addb933a65be77092b34a7e77d12fe8611a61e00ee6848b85091ecca9d1e508",
	"0x00d7540dcd268a845c10ae18d1de933cf638ff5425f0afff7935628e299d1791",
	"0x140c0e42687e9ead01b2827a5664ca9c26fedde4acd99db1d316939d20b82c0e",
	"0x2f0c3a115d4317d191ba89b8d13d1806c20a0f9b24f8c5edc091e2ae56565984",
	"0x0c4ee778ff7c14553006ed220cf9c81008a0cff670b22b82d8c538a1dc958c61",
	"0x1704f2766d46f82c3693f00440ccc3609424ed26c0acc66227c3d7485de74c69",
	"0x2f2d19cc3ea5d78ea7a02c1b51d244abf0769c9f8544e40239b66fe9009c3cfa",
	"0x1ae03853b75fcaba5053f112e2a8e8dcdd7ee6cb9cfed9c7d6c766a806fc6629",
	"0x0971aabf795241df51d131d0fa61aa5f3556921b2d6f014e4e41a86ddaf056d5",
	"0x1408c316e6014e1a91d4cf6b6e0de73eda624f8380df1c875f5c29f7bfe2f646",
	"0x1667f3fe2edbe850248abe42b543093b6c89f1f773ef285341691f39822ef5bd",
	"0x13bf7c5d0d2c4376a48b0a03557cdf915b81718409e5c133424c69576500fe37",
	"0x07620a6dfb0b6cec3016adf3d3533c24024b95347856b79719bc0ba743a62c2c",
	"0x1574c7ef0c43545f36a8ca08bdbdd8b075d2959e2f322b731675de3e1982b4d0",
	"0x269e4b5b7a2eb21afd567970a717ceec5bd4184571c254fdc06e03a7ff8378f0",
];
pub const MDS_ENTRIES: [[&str; 2]; 2] = [
	[
		"0x066f6f85d6f68a85ec10345351a23a3aaf07f38af8c952a7bceca70bd2af7ad5",
		"0x2b9d4b4110c9ae997782e1509b1d0fdb20a7c02bbd8bea7305462b9f8125b1e8",
	],
	[
		"0x0cc57cdbb08507d62bf67a4493cc262fb6c09d557013fff1f573f431221f8ff9",
		"0x1274e649a32ed355a31a6ed69724e1adade857e86eb5c3a121bcd147943203c8",
	],
];

pub fn get_rounds_poseidon_bn254_x5_2<F: PrimeField>() -> Vec<F> {
	parse_vec(ROUND_CONSTS.to_vec())
}
pub fn get_mds_poseidon_bn254_x5_2<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(MDS_ENTRIES.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
}

pub fn get_poseidon_bn254_x5_2<F: PrimeField>() -> PoseidonParameters<F> {
	let rounds = get_rounds_poseidon_bn254_x5_2();
	let mds = get_mds_poseidon_bn254_x5_2();
	PoseidonParameters::<F>::new(rounds, mds, FULL_ROUNDS, PARTIAL_ROUNDS, WIDTH, SBOX)
}
