<template>
  <div>
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
export default {
  name: "Output",

  props: {
    codes: { type: Object },
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
</style>
