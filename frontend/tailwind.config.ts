import type { Config } from 'tailwindcss';

const config: Config = {
    // Tailwinde がどのファイルをスキャンするか
    // Tailwindは実際に使われているクラス名だけを生成するため、使用されているファイルを指定する必要がある
    content: [
        './index.html',
        './src/**/*.{ts,tsx}',
    ],
    theme: {
        extend: {},
    },
    plugins: [],
};

export default config;