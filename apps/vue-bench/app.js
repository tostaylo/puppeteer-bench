// import ATable from './table.js';

export default {
	name: 'App',
	data() {
		return {
			counter: 0,
			tableLength: 0,
			words: [
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
			],
		};
	},

	methods: {
		handleCreateClear: function (len) {
			this.tableLength = len;
			this.counter = this.counter += 1;
		},
		getIndex: function (num) {
			return num <= 14 ? num + 14 + this.counter : num + this.counter;
		},
	},

	// components: {
	// 	ATable,
	// },
	template: `
    <div id="main" class="main">
      <header>
	      <h1>vue-bench</h1>
        <button id="create1000" v-on:click="() => handleCreateClear(1000)">CreateK</button>
        <button id="create10000"v-on:click="() => handleCreateClear(10000)">CreateK</button>
        <button id="clear" v-on:click="() => handleCreateClear(0)">Clear</button>
      </header>
      <table v-if="tableLength > 0">
        <tr v-for="(n, index) in tableLength" :key=n>
          <td>{{n}}</td><td>{{words[getIndex(n) % 12]}} {{words[getIndex(n) % 13]}} {{words[getIndex(n) % 14]}}</td>
        </tr>
      </table>
    </div>
  `,
};