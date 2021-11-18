<template>
  <div>
    <!-- show the encoded text -->
    <p id="text">
      Encoded text: <strong>{{ codes.encoded_text }}</strong>
    </p>

    <!-- visualize the huffman tree -->
    <div id="tree">
      <h1>Huffman Tree</h1>
      <p><i>A left edge encode value is 0, 1 for a right branch.</i></p>
      <div>
        <blocks-tree
          :data="treeData"
          :horizontal="treeOrientation == '1'"
          :collapsable="true"
        ></blocks-tree>
      </div>
    </div>

    <!-- provide the frequency distribution -->
    <table>
      <tr>
        <th>Character</th>
        <th>Frequency</th>
        <th>Huffman Code</th>
      </tr>
      <tr v-for="code in codes.codes.huffman_codes" :key="code.character">
        <td>{{ code.character }}</td>
        <td>{{ code.frequency }}</td>
        <td>{{ code.huffman_code }}</td>
      </tr>
    </table>
  </div>
</template>

<script>
import { ref, reactive } from "vue";

export default {
  name: "Output",

  props: {
    codes: { type: Object },
  },

  setup(props) {
    let selected = ref([]);
    let treeOrientation = ref("0");

    function generate_tree(tree, id) {
      if (Object.prototype.hasOwnProperty.call(tree, "Node")) {
        let node = tree.Node;
        // left
        let left = generate_tree(node.left, id + 1);
        // right
        let right = generate_tree(node.right, id + 2);

        return {
          label: node.freq,
          expand: true,
          some_id: id,
          children: [left, right],
        };
      }

      let leaf = tree.Leaf;
      return { label: `${leaf.freq}, '${leaf.value}'`, some_id: id };
    }

    let treeData = reactive(generate_tree(props.codes.tree));

    const toggleSelect = (node, isSelected) => {
      isSelected
        ? selected.value.push(node.some_id)
        : selected.value.splice(selected.value.indexOf(node.some_id), 1);
      if (node.children && node.children.length) {
        node.children.forEach((ch) => {
          toggleSelect(ch, isSelected);
        });
      }
    };

    return {
      treeData,
      selected,
      toggleSelect,
      treeOrientation,
    };
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

#tree,
h1 {
  width: 100%;
  text-align: center;
}

#text {
  padding: 50px;
  width: 100%;
  text-align: center;
}
</style>
