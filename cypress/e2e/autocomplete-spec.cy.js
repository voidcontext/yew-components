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
  
  it('clear autocomplete items if we delete enough chars', () => {
    cy.visit('http://localhost:9001')
    cy.get('input[type=text]')
      .type("foo")

    cy.get('li.autocomplete-item').should('have.length', 2)
    
    cy.get('input[type=text]')
      .type("{backspace}")

    cy.get('li.autocomplete-item').should('have.length', 0)
  })
  
  it('should hihglight selected item', () => {
    cy.visit('http://localhost:9001')
    cy.get('input[type=text]')
      .type("foo{downArrow}")

    cy.get('li.autocomplete-item.selected').should('have.text', "foo")
    
    cy.get('input[type=text]')
      .type("{downArrow}")

    cy.get('li.autocomplete-item.selected').should('have.text', "foobar")

    cy.get('input[type=text]')
      .type("{upArrow}")

    cy.get('li.autocomplete-item.selected').should('have.text', "foo")
  })

  // Skipping this test since the `type`'s behavour is not the same is as typing in the browser so
  // the up and down arrows don't move the cursor the beginning and end of the text
  it.skip('should not move the cursor within the input when selecting items', () => {
    cy.visit('http://localhost:9001')
    cy.get('input[type=text]')
      .type("foo{downArrow}{downArrow}{upArrow}bar", {delay: 1000})

    cy.get('input[type=text]')
      .should('have.value', "foobar")
    
    cy.get('li.autocomplete-item').should('have.length', 1)
  })
})