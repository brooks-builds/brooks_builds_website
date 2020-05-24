<template>
  <v-app-bar app color="primary" dark class="d-flex justify-space-around mb-6">
      <v-btn
        text
      >
        <span class="mr-2">Home</span>
      </v-btn>
      <v-btn text class="mr-2">
        <span class="mr-2">Recent Videos</span>
      </v-btn>
      <v-btn text class="mr-2">
        <span class="mr-2">Chat Commands</span>
      </v-btn>
      <v-btn text class="mr-2">
        <span class="mr-2">Current Projects</span>
      </v-btn>
      <v-btn text class="mr-2">
        <span class="mr-2">Project Suggestions</span>
      </v-btn>
    </v-app-bar>
</template>

<script>
export default {
  name: "Navigation"
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

</style>
