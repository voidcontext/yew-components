describe('autocomplete spec', () => {
  it('contains input field', () => {
    cy.visit('http://localhost:9001')
    cy.get('input[type=text]')
  })
})