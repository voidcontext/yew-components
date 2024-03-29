
["plain", "bulma"].forEach((theme) => {

  describe(`autocomplete spec - simple ${theme}`, () => {
  
    it('demo page contains an input field', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('input[type=text]')
    })

    it('updates the input field with the type value', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("foobar")
        .should('have.value', 'foobar')
    })

    it('generates completion options when input lowercased', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("uni")

      cy.get('#single-select .autocomplete-item').should('have.length', 3)
    })
    it('generates completion options when input capitalized', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("Uni")

      cy.get('#single-select .autocomplete-item').should('have.length', 3)
    })
  
    it('clear autocomplete items if we delete enough chars', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("uni")

      cy.get('#single-select .autocomplete-item').should('have.length', 3)
    
      cy.get('#single-select input[type=text]')
        .type("{backspace}")

      cy.get('#single-select .autocomplete-items').should('not.exist')
      cy.get('#single-select .autocomplete-item').should('have.length', 0)
    })
  
    it('should hihglight selected item', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("uni{downArrow}")

      cy.get('#single-select .autocomplete-item.highlighted').should('have.text', "United Arab Emirates")
    
      cy.get('#single-select input[type=text]')
        .type("{downArrow}")

      cy.get('#single-select .autocomplete-item.highlighted').should('have.text', "United Kingdom")

      cy.get('#single-select input[type=text]')
        .type("{upArrow}")

      cy.get('#single-select .autocomplete-item.highlighted').should('have.text', "United Arab Emirates")
    })

    // Skipping this test since the `type`'s behavour is not the same is as typing in the browser so
    // the up and down arrows don't move the cursor the beginning and end of the text
    it.skip('should not move the cursor within the input when selecting items', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("uni{downArrow}{downArrow}{upArrow}ted", {delay: 1000})

      cy.get('#single-select input[type=text]')
        .should('have.value', "united")
    
      cy.get('#single-select .autocomplete-item').should('have.length', 3)
    })
  
    it('should select hihglighted item', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("united{downArrow}{downArrow}{enter}")

      cy.get('#single-select p').should('have.text', "Selected country: United Kingdom")
    })

    it('should clean the input field after selection', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("united{downArrow}{downArrow}{enter}")

      cy.get('#single-select p').should('have.text', "Selected country: United Kingdom")
      cy.get('#single-select input').should('have.value', "")
    })

    it('should clean the items after selection', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("united{downArrow}{downArrow}{enter}")

      cy.get('#single-select p').should('have.text', "Selected country: United Kingdom")
      cy.get('#single-select .autocomplete-item').should('have.length', 0)
    })

    it('should select clicked item', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("united")

      cy.get(".autocomplete-item:nth(1)").click()

      cy.get('#single-select p').should('have.text', "Selected country: United Kingdom")
    })
  
    it('should hide the list of selected items', () => {
      cy.visit(`http://localhost:9001/${theme}/simple`)
      cy.get('#single-select input[type=text]')
        .type("united{downArrow}{downArrow}{enter}")

      cy.get('#single-select ul.selected-items').should('not.exist')
    })
  }) 
});
