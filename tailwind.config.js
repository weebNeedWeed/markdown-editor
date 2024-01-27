module.exports = {
  	content: ["src/**/*.rs", "index.html"],
  	theme: {
  		extend: {
			colors: {
				skin: {
                    primary: "var(--color-primary)",
                    typography: "var(--color-typography)",
                    buttons: "var(--color-buttons)",
                }
			},
            backgroundImage: {
                skin: "var(--background)"
            }
		},
  	},
  	plugins: [],
}

