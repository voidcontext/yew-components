describe('autocomplete spec', () => {
  it('demo page contains an input field', () => {
    cy.visit('http://localhost:9001')
    cy.get('input[type=text]')
  })

  it('updates the input field with the type value', () => {
    cy.visit('http://localhost:9001')
    cy.get('input[type=text]')
      .type("foobar")
      .should('have.value', 'foobar')
  })

  it('generates completion options', () => {
    cy.visit('http://localhost:9001')
    cy.get('input[type=text]')
      .type("foo")

    cy.get('li.autocomplete-item').should('have.length', 2)
  })
})