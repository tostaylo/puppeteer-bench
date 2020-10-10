export default `
html,
body {
	margin: 0;
	font-family: Helvetica, Arial, sans-serif;
	box-sizing: border-box;
	-webkit-text-size-adjust: 100%;
	color: white;
	background: black;
}

body {
	padding: 20px;
	display: flex;
	flex-direction: column;
	justify-content: flex-start;
	align-items: center;
	min-height: 120vh;
}

#root {
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	width: 100%;
}

.main {
	display: flex;
	flex-direction: column;
	justify-content: center;
	width: 70%;
}

header {
	display: flex;
	justify-content: space-evenly;
	align-items: center;
}

table {
  align-self: center;
	border: 1px solid rgb(236, 235, 235);
	border-collapse: collapse;
	width: 300px;
}

th,
td {
	text-align: left;
	padding: 8px;
}

tr:nth-child(even) {
	background: rgb(102, 102, 102);
}

button {
	cursor: pointer;
	padding: 1em;
	background: rgb(102, 102, 102);
	color: white;
	outline: none;
	border: none;
	border-radius: 3px;
	margin-left: 20px;
}
`;
