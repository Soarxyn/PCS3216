from sisprog import assemble, link, print_debug

if __name__ == "__main__":
    assemblyResult = assemble("ex.qck", "ex.bdc")
    if assemblyResult == "Assembly successful":
        linkingResult = link(["ex.bdc"], "ex.fita")
        if linkingResult == "Linking successful":
            print_debug("ex.fita")
        else:
            print(linkingResult)
    else:
        print(assemblyResult)
