export default class Erlpack {
	public static pack(data: string) {
		Deno.openPlugin(
			Deno.cwd() + '\\src\\util\\erlpack\\target\\debug\\erlpack.dll'
		);

		// @ts-ignore

		const { pack } = Deno.core.ops();

		// @ts-ignore
		const res = Deno.core.dispatch(pack, new TextEncoder().encode(data));

		console.log('Packed:', new TextDecoder().decode(res));
	}
}
