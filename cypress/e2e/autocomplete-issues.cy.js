["plain", "bulma"].forEach((theme) => {

  describe(`autocomplete - Issue  #001 ${theme}`, () => {
    it("should update internal state when props are updated", () => {
      cy.visit(`http://localhost:9001/${theme}/issue-001`)
      cy.get('#single-select input[type=text]')
        .type("simple-tag{downArrow}")

      cy.get('#single-select .autocomplete-item').should('have.length', 1)
      cy.get('#single-select .autocomplete-item.highlighted').should('have.text', "simple-tag")
    
      cy.get('#single-select input[type=text]')
        .type("{enter}")

      cy.get('#single-select input[type=text]')
        .type("simple{downArrow}")

      cy.get('#single-select .autocomplete-item').should('have.length', 2)
      cy.get('#single-select .autocomplete-item.highlighted').should('have.text', "simple")

      cy.get('#single-select input[type=text]')
        .type("{downArrow}")

      cy.get('#single-select .autocomplete-item.highlighted').should('have.text', "simple-tag")
    })
  })
})
