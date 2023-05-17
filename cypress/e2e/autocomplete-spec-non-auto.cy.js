describe('autocomplete spec - non auto', () => {
  it('demo page should contain a seach button', () => {
    cy.visit('http://localhost:9001/non-auto')
    cy.get('input[type=button]').should('have.value', 'Search')
  })

  it("shouldn't offer autocomplete options", () => {
    cy.visit('http://localhost:9001/non-auto')
    cy.get('#single-select input[type=text]')
      .type("uni")

    cy.get('#single-select ul.autocomplete-items').should('not.exist')
  })
  
  it("clickink on search should display autocomplete options", () => {
    cy.visit('http://localhost:9001/non-auto')
    cy.get('#single-select input[type=text]')
      .type("uni")
    
    cy.get('input[type=button]').click()

    cy.get('#single-select li.autocomplete-item').should('have.length', 3)
  })
})