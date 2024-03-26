module.exports = {
  	content: ["src/**/*.rs", "index.html"],
  	theme: {
  		extend: {
			colors: {
				skin: {
                    primary: "var(--color-primary)",
                    typography: "var(--color-typography)",
                    buttons: "var(--color-primary)",
                    background: "var(--color-background)",
                }
			},
		},
  	},
  	plugins: [
        require('@tailwindcss/typography') 
    ],
}

