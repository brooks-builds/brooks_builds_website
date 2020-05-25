<template>
  <v-container>
    <v-row class="justify-center">
      <h1>Chat Commands</h1>
    </v-row>
    <v-row>
      <p>
        The following are the commands that you can use at Brooks' Twitch live
        stream at
        <a href="https://www.twitch.tv/brookzerker"
          >https://www.twitch.tv/brookzerker</a
        >.
      </p>
    </v-row>
    <v-row class="justify-center">
      <v-data-table :headers="headers" :items="chatCommands"></v-data-table>
    </v-row>
  </v-container>
</template>

<script>
export default {
  name: "ChatCommand",
  computed: {
    chatCommands() {
      return this.$store.state.chatCommands;
    },
  },
  data() {
    return {
      headers: [
        { text: "command", value: "command" },
        { text: "response", value: "response" },
        { text: "times used", value: "used" },
      ],
    };
  },
};
</script>

<style scoped></style>
