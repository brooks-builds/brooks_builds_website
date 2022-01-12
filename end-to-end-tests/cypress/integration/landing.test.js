describe("Brooks Builds landing page", () => {
  it("should load the hello world", () => {
    cy
      .visit("/")
      .get("[data-test-hello]")
      .should("contain", "Hello World");
  })
})