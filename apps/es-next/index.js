const words = [
	'There',
	'High',
	'Lizards',
	'Sappy',
	'Wreck',
	'Fairly',
	'Barking',
	'Lurching',
	'Carbs',
	'Flat',
	'Hard',
	'Sad',
	'Butterfly',
	'Bandana',
];

let counter = 0;

(function run() {
	const root = document.getElementById('root');
	const header = document.createElement('header');
	const main = document.createElement('div');

	main.id = 'main';
	main.className = 'main';

	root?.appendChild(main);
	main.appendChild(header);

	const h1 = document.createElement('h1');
	h1.innerText = 'es-next-bench';

	const k_button = document.createElement('button');
	k_button.id = 'create1000';
	k_button.innerText = 'Create K';

	const ten_k_button = document.createElement('button');
	ten_k_button.id = 'create10000';
	ten_k_button.innerText = 'Create 10K';

	const clear_button = document.createElement('button');
	clear_button.id = 'clear';
	clear_button.innerText = 'Clear';

	k_button.addEventListener('click', () => createTable(1000));
	ten_k_button.addEventListener('click', () => createTable(10000));
	clear_button.addEventListener('click', () => {
		document.querySelector('table')?.remove();
	});

	header.appendChild(h1);
	header.appendChild(k_button);
	header.appendChild(ten_k_button);
	header.appendChild(clear_button);
})();

function createTable(rows) {
	const oldTable = document.querySelector('table');
	oldTable?.parentNode?.removeChild(oldTable);

	if (rows > 0) {
		const table = document.createElement('table');
		const tableBody = document.createElement('tbody');

		for (let i = 0; i < rows; i++) {
			const idx = i <= 14 ? i + 14 + counter : i + counter;
			const row = document.createElement('tr');
			const data1 = document.createElement('td');
			const data2 = document.createElement('td');

			const data1Text = document.createTextNode((1 + i).toString());
			const data2Text = document.createTextNode(`${words[idx % 12]} ${words[idx % 13]} ${words[idx % 14]}`);

			data1.appendChild(data1Text);
			data2.appendChild(data2Text);
			row.appendChild(data1);
			row.appendChild(data2);
			tableBody.appendChild(row);
		}

		const root = document.getElementById('main');
		table.appendChild(tableBody);
		root?.appendChild(table);
	}
	counter += counter + 1;
}