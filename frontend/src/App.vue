<template>
  <div>
    <Input @compressed-data="setOutput" />

    <div v-if="compressed_data">
      <Output :codes="compressed_data" />
    </div>
  </div>
</template>

<script>
import Input from "./components/Input.vue";
import Output from "./components/Output.vue";

export default {
  name: "App",

  components: {
    Input,
    Output,
  },

  data() {
    return { compressed_data: null };
  },

  methods: {
    setOutput(payload) {
      console.log(payload);
      this.compressed_data = payload;
    },
  },
};
</script>

<style scoped></style>
