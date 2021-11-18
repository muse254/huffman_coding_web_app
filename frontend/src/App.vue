<template>
  <div>
    <div id="logo">
      <img src="./assets/logo.png" />
    </div>

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
      this.compressed_data = payload;
    },
  },
};
</script>

<style scoped>
#logo {
  width: 100%;
  text-align: center;
  padding: 20px;
}

#logo img {
  width: 150px;
  height: 150px;
}
</style>
