
["plain", "bulma"].forEach((theme) => {

  describe(`autocomplete spec - non auto ${theme}`, () => {
    it('demo page should contain a seach button', () => {
      cy.visit(`http://localhost:9001/${theme}/nonauto`)
      cy.get('input[type=button]').should('have.value', 'Search')
    })

    it("shouldn't offer autocomplete options", () => {
      cy.visit(`http://localhost:9001/${theme}/nonauto`)
      cy.get('#single-select input[type=text]')
        .type("uni")

      cy.get('#single-select .autocomplete-items').should('not.exist')
    })
  
    it("clickink on search should display autocomplete options", () => {
      cy.visit(`http://localhost:9001/${theme}/nonauto`)
      cy.get('#single-select input[type=text]')
        .type("uni")
    
      cy.get('input[type=button]').click()

      cy.get('#single-select .autocomplete-items').should('exist')
      cy.get('#single-select .autocomplete-item').should('have.length', 3)
    })
  })
})