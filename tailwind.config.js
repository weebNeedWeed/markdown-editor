module.exports = {
  	content: ["src/**/*.rs", "index.html"],
  	theme: {
  		extend: {
			colors: {
				skin: {
                    primary: "var(--color-primary)",
                    typography: "var(--color-typography)",
                    buttons: "var(--color-buttons)",
                    secondary: "var(--color-secondary)",
                    background: "var(--color-background)",
                }
			},
		},
  	},
  	plugins: [],
}

