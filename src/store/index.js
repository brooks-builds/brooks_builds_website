import Vue from "vue";
import Vuex from "vuex";
import Axios from "axios";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    colors: {
      youtube: "#ff0000",
      github: "#24292e",
      meetup: "#f13a59",
    },
    chatCommands: [],
  },
  mutations: {
    setChatCommands(state, commands) {
      Vue.set(state, "chatCommands", commands);
    },
  },
  actions: {
    fetchAllChatCommands: async function({ commit }) {
      try {
        const result = await Axios.get(
          "https://chatbot.brooksbuilds.com/api/v1/commands"
        );

        const commands = result.data.map((command) => {
          if (command.response == "IGNORE") command.response = "";

          return command;
        });

        console.log("got commands", commands);
        commit("setChatCommands", commands);
      } catch (error) {
        console.error(error);
      }
    },
  },
  modules: {},
});
