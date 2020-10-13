const WebC = `<main-component/>`;

export type Config = { dirName: string; framework: string; src: string; webComponent?: typeof WebC };

function createConfig(name: string, isWebComp: boolean): Config {
	return {
		dirName: name,
		framework: name,
		src: `./apps/${name}/index.js`,
		webComponent: isWebComp ? WebC : undefined,
	};
}

export const configs: Config[] = [
	createConfig('web-component', true),
	createConfig('rust-wasm', false),
	createConfig('rust-fel', false),
	createConfig('react', false),
	createConfig('vue', false),
	createConfig('es-next', false),
];
