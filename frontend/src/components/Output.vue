<template>
  <div>
    <h1>HUFFMAN CODING</h1>
    <!-- provide the frequency distribution -->
    <div>
      <h2>Distribution Table</h2>
      <table>
        <tr>
          <th>Character</th>
          <th>Frequency</th>
          <th>Huffman Code</th>
        </tr>
        <tr v-for="code in codes.codes.huffman_codes" :key="code.character">
          <td>'{{ code.character }}'</td>
          <td>{{ code.frequency }}</td>
          <td>{{ code.huffman_code }}</td>
        </tr>
      </table>
    </div>

    <!-- visualize the huffman tree -->
    <div id="tree">
      <h2>Huffman Tree</h2>
      <p><i>A left edge encode value is 0 else 1 for a right branch.</i></p>
      <div>
        <blocks-tree
          :data="treeData"
          :horizontal="treeOrientation == '1'"
          :collapsable="true"
        ></blocks-tree>
      </div>
    </div>

    <p>
      The Huffman Code is: <strong>{{ codes.encoded_text }}</strong>
    </p>

    <h1>HUFFMAN DECODING</h1>

    <p>
      The Huffman Tree can be decoded recursively using the encoded text to
      reconstruct the original text. We traverse the tree such that if we
      encounter a 0, we move an edge to the left & to the right if 1 was
      encountered.
    </p>
    <p>
      If we encounter a leaf, we print out the symbol it holds to output. We
      move back to the root of the Huffman Tree and decode again starting from
      the current offset. Doing this recursively decodes the compressed encoded
      text.
    </p>

    <div id="button">
      <button v-on:click="decode_text">
        Decode The Huffman Codes with Huffman Tree
      </button>
    </div>

    <div v-if="decoded_text">
      <h2>Decoded Text</h2>
      <p>
        <strong>{{ decoded_text }}</strong>
      </p>
    </div>
  </div>
</template>

<script>
export default {
  name: "Output",

  props: {
    codes: { type: Object },
  },

  data() {
    return { decoded_text: "" };
  },

  methods: {
    generate_tree: function(tree, id) {
      if (Object.prototype.hasOwnProperty.call(tree, "Node")) {
        let node = tree.Node;
        // left
        let left = this.generate_tree(node.left, id + 1);
        // right
        let right = this.generate_tree(node.right, id + 2);

        return {
          label: node.freq,
          expand: true,
          some_id: id,
          children: [left, right],
        };
      }
      let leaf = tree.Leaf;
      return { label: `${leaf.freq}, '${leaf.value}'`, some_id: id };
    },

    decode_text() {
      console.log(this.codes);
      // send data to server
      const request_options = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(this.codes),
      };

      fetch("http://localhost:8000/decompress", request_options)
        .then(async (response) => {
          const data = await response.json();

          if (!response.ok) {
            // get error message from body or default to response status
            const error =
              data.error | (data && data.message) || response.status;
            return Promise.reject(error);
          }

          this.decoded_text = data.text;
        })
        .catch((error) => {
          console.error("There was an error!", error);
          alert(`${error}`);
        });
    },
  },

  computed: {
    treeData() {
      return this.generate_tree(this.codes.tree);
    },
  },
};
</script>

<style scoped>
table {
  margin: 25px auto;
  border-collapse: collapse;
  border: 1pc solid #eee;
  border-radius: 5px;
  box-shadow: 0px 0px 20px rgba(0, 0, 0, 0.1), 0px 10px 20px rgba(0, 0, 0, 0.05),
    0px 20px 20px rgba(0, 0, 0, 0.05), 0px 30px 20px rgba(0, 0, 0, 0.05);
}

tr:hover {
  background: #f4f4f4;
}

td {
  color: #555;
}

th,
td {
  color: #999;
  border: 1px solid #eee;
  padding: 12px 35px;
  border-collapse: collapse;
}
th {
  background: #00cccc;
  color: #fff;
  text-transform: uppercase;
  font-size: 12px;
}

th {
  border-right: none;
}

#button,
#tree,
#download,
h1,
p,
h2 {
  width: 100%;
  text-align: center;
}

#download {
  margin-top: 50px;
}

#text {
  padding: 50px;
  width: 100%;
  text-align: center;
}
</style>
