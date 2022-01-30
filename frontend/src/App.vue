<template>
  <div>
    <div id="logo">
      <img src="./assets/logo.png" />
    </div>

    <!-- <InputToggle /> -->
    <div>
      <TextInput @compressed-data="setOutput" />
    </div>

    <div v-if="compressed_data">
      <Output :codes="compressed_data" />
    </div>
  </div>
</template>

<script>
import Output from "./components/Output.vue";
import TextInput from "./components/TextInput.vue";

export default {
  name: "App",

  components: {
    Output,
    TextInput,
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

#nav {
  display: flex;
  justify-content: center;
}
</style>
